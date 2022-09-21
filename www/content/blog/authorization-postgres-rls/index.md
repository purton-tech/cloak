+++
title = "Using Row Level Security for Authorization with Postgres"
date = 2022-09-19
+++

## What is Authorization in this context

Whenever you've build a web applciation you've probable writtent your database queries something like the following so that suers only see their own data and not data from other users.

```sql
SELECT id, name FROM products WHERE user_id = 123
```

By filtering your users you are actually performing [Authorization](https://en.wikipedia.org/wiki/Authorization) at the application level. You've also implemented a version of [Multi Tenancy](https://en.wikipedia.org/wiki/Multitenancy). Multi tenancy is how we divide a database between different users or organisations.

So with one query you've done a lot already. But can we do better?

## What does RLS look like

The follwoign  policy gives us the same functionality as the query ablove.

```sql
CREATE POLICY multi_tenancy_policy ON products FOR ALL TO application
USING (
    user_id IN (SELECT get_user())
);
```

So now to query the database from our application we just issue.

```sql
SELECT id, name FROM products
```

## How does RLS Help?

[Row level security](https://www.postgresql.org/docs/current/ddl-rowsecurity.html) allows us to perform our Authorization at the database level. By doing sow e can get the following benefits.

1. RLS allows us to define policies using SQL. We can take a skill we already have and use it to do good elsewhere.
1. By placing the Authorization logic at the database level we simplify our application queries.
1. Having one place for Authorization logic means that if we have more than one applciation pointing at our dfatase the logic gets re-used.
1. We can implement a form of RBAC and enforce it in one place.
1. RLS allows us to enforce policies such as gibe me the data this user is allowed to see. This authorization logic is not supported or hard to perform in some of the Authorization libraries such as [Casbin](https://casbin.org/).

Let's take a look at some common use cases for RLS and how we can add them to our applications.

## Folow along with docker


## Row Level Security Setup


```sql
CREATE FUNCTION current_app_user() RETURNS INTEGER AS 
$$ 
    SELECT
        current_setting(
            'row_level_security.user_id',
            false
        )::integer 
$$ LANGUAGE SQL;
COMMENT ON FUNCTION current_app_user IS 
    'These needs to be set by the application before accessing the database.';
```

## Multi Teancy.

```sql
ALTER TABLE audit_trail ENABLE ROW LEVEL SECURITY;
```

## Implementing Role Based Access Control

## Enforcing RBAC with RLS

## Performance Issues

## Using RLS to counter 