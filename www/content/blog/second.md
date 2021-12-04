+++
title = "Another pages"
date = 2021-11-08
+++

The vast majority of software has secrets. While this makes that software sound mysterious, like a femme fatale in a noir film, most software secrets are mundane. But that doesn’t mean those secrets aren’t important! Software secrets management can often be the difference between someone leaking confidential customer data or keeping that data secure. Understanding what secrets management is, and how you should approach it in your application, is a critical step in application maturity. In this post, we’re going to talk about what software secrets are, how you shouldn’t secure them, and what functionality you want in your secrets management approach.

What Are Software Secrets?
As we said, secrets are often mundane. Even basic static websites need to keep secrets. For most applications, secrets take the form of database passwords and encryption private keys. Even for static sites, secrets can take the form of keys used to connect to deployment systems or passwords for the web hosting server. As a developer, it’s your responsibility to protect these secrets. If someone malicious gained access to your application secrets, they’d gain access to whatever those secrets protect. Like we already mentioned, this might mean leaking your database data. If the secret is instead the password to a hosting server, they could replace your website with malware or deface.