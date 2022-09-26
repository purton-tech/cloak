+++
title = "Implementing Authorization with Postgres Row Level Security (RLS)"
date = 2022-09-19
+++

## Let's take a simple example of Authorization

Whenever you've build a web application you've probable written your database queries something like the following so that users only see their own data and not data from other users.

```sql
SELECT id, name FROM products WHERE user_id = 123
```

By filtering your users you are actually performing [Authorization](https://en.wikipedia.org/wiki/Authorization) at the application level. You've also implemented a version of [Multi Tenancy](https://en.wikipedia.org/wiki/Multitenancy). Multi tenancy is how we divide a database between different users or organisations.

So with one query you've done a lot already. But can we do better?

## How can we do this with Row Level Security?

The following  policy gives us the same functionality as the query above.

```sql
CREATE POLICY multi_tenancy_policy ON products FOR ALL TO application
USING (
    user_id IN (SELECT current_app_user())
);
```

So now to query the database from our application we could just issue.

```sql
SELECT id, name FROM products
```

And the RLS policy will ensure only the rows we are allowed to see are returned.

## How does RLS Help?

[Row level security](https://www.postgresql.org/docs/current/ddl-rowsecurity.html) allows us to perform our Authorization at the database level. By doing so we can get the following benefits.

1. RLS allows us to define policies using SQL. We can take a skill we already have and reuse it to fix another problem.
1. By placing the Authorization logic at the database level we simplify our application queries.
1. Having one place for Authorization logic means that if we have more than one application pointing at our database the logic gets re-used.
1. We can implement a form of RBAC and enforce it in one place.
1. RLS allows us to enforce policies such as give me the data this user is allowed to see. This authorization logic is not supported or hard to perform in some of the Authorization libraries such as [Casbin](https://casbin.org/).

Let's take a look at some common use cases for RLS and how we can add them to our applications.

## Follow along with docker

If you open two terminal windows and issue the following docker commands we can setup a simple demonstration.

```sh
# Then run postgres
docker run -it --rm --name postgres-test -e POSTGRES_PASSWORD=postgresPW postgres
```

In another terminal connect to out test Postgres

```sh
docker exec -ti postgres-test psql -U postgres
```

And you'll hopefully get a PSQL command prompt.

```
psql (14.5 (Debian 14.5-1.pgdg110+1))
Type "help" for help.

postgres=#
```

## Setup up some test data and a helper function

Create a table with some data

```sql
CREATE TABLE products AS SELECT id, floor(random() * 10 + 1)::int as user_id, md5(random()::text) AS name FROM generate_Series(1,5000) id;
```

The following helper function retrieves the `user_id` from a Postgres variable.

```sql
CREATE OR REPLACE FUNCTION current_app_user() RETURNS INTEGER AS 
$$ 
    SELECT
        current_setting(
            'row_level_security.user_id',
            false
        )::integer 
$$ LANGUAGE SQL STABLE;
COMMENT ON FUNCTION current_app_user IS 
    'row_level_security.user_id needs to be set by the application before accessing the database.';
```

This is necessary as we will have our own users table and we won't create Postgres roles for each user in our system.

## An Example of Multi Tenancy

Add the following policy to your Postgres session.

```sql
CREATE POLICY multi_tenancy_policy ON products FOR ALL TO application
USING (
    user_id IN (SELECT current_app_user())
);
```

To use the policy we have to switch on RLS for the tale. 

```sql
ALTER TABLE products ENABLE ROW LEVEL SECURITY;
```

We also have to create a new role. Up until now we've been using a role that has bypass RLS as a property. We'll create a new role called *application* and switch to it.

```sql
CREATE ROLE application;
GRANT SELECT ON products TO application;
```

Switch to the role.

```sql
SET role application;
```

Try and access the database.

```sql
SELECT COUNT(*) FROM products;
```

You should get an error

```
ERROR:  unrecognized configuration parameter "row_level_security.user_id"
CONTEXT:  SQL function "current_app_user" statement 2
```

We need to set the `user_id` so that the current_app_user function knows who we are referring to.

```sql
SET SESSION row_level_security.user_id TO 5;
```

N.B. In your application you will need to set this variable using `SET LOCAL` and this will need to be apart of a transaction.

Now try again.

```sql
SELECT COUNT(*) FROM products;
```

You should get around about 500 results back from a total of 5000.

This shows that the RLS policy has kicked in and restricted what the user an see.

## Performance Issues

There is a gotcha that you need to be aware of. Postgres RLS is not sing any indexes we create. Add the following index then run an explain plan.

```sql
CREATE INDEX idx_product_user ON products(user_id);
```

```sql
EXPLAIN ANALYZE VERBOSE SELECT COUNT(*) from products;
```

The *Seq Scan on public.products* section shows us the index is not used.

```
 Aggregate  (cost=116.01..116.02 rows=1 width=8) (actual time=1.304..1.305 rows=1 loops=1)
   Output: count(*)
   ->  Seq Scan on public.products  (cost=0.26..109.76 rows=2500 width=0) (actual time=0.200..1.257 rows=475 loops=1)
         Output: products.id, products.user_id, products.name
         Filter: (hashed SubPlan 1)
         Rows Removed by Filter: 4525
         SubPlan 1
           ->  Result  (cost=0.00..0.26 rows=1 width=4) (actual time=0.178..0.178 rows=1 loops=1)
                 Output: current_app_user()
 Planning Time: 0.103 ms
 Execution Time: 1.348 ms
(11 rows)
```

This is because Postgres run the RLS policies **AFTER** the results have been pulled from the database. So to get our index to trigger we need to put our some of our policy into the SQL call.

```sql
EXPLAIN ANALYZE VERBOSE SELECT COUNT(*) from products WHERE user_id = 6;
```

Here we can see, by adding the WHERE clause we were able to get the index to trigger.

```
Aggregate  (cost=14.30..14.31 rows=1 width=8) (actual time=0.144..0.144 rows=1 loops=1)
   Output: count(*)
   ->  Index Only Scan using idx_product_user on public.products  (cost=0.28..13.05 rows=501 width=0) (actual time=0.037..0.115 rows=501 loo
ps=1)
         Output: user_id
         Index Cond: (products.user_id = 6)
         Heap Fetches: 0
 Planning Time: 0.272 ms
 Execution Time: 0.162 ms
(8 rows)
```

So as far as I can tell, you need to be very careful how you from the read SQL queries with RLS or you will get a performance hit.

## Other Articles to look at

* [Using PostgreSQL row level security (RLS) to authorize READ queries for your application’s users.](https://medium.com/@bartels/using-postgresql-row-level-security-rls-to-authorize-read-queries-for-your-applications-users-a2838d2afb92) and especially this comment which tries to fix some of the issues in the article. [Comment](https://medium.com/@ethanresnick/there-are-a-few-faster-ways-that-i-know-of-to-handle-the-third-case-with-rls-9d22eaa890e5)

## Conclusion

Postgres RLS is very useful but you're going to have to to keep in mind the following.

* Make use of *EXPLAIN ANALYZE* a lot to ensure your policies are using the indexes when required.
* If you create functions to use in your policies mark them as [STABLE](https://www.postgresql.org/docs/current/xfunc-volatility.html) so they get cached during the transaction.

Above all, have fun.