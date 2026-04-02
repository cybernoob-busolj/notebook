---
os: linux
status: WIP
tags:
  - machines/medium/neo-forge
aliases:
---
# Resolution summary

>[!summary]
>- Recon
>- 

## Improved skills

- Skill 1
- Skill 2

## Used tools

- nmap
- gobuster


---

# Information Gathering

Scanned all TCP ports:

```bash
22/tcp open  ssh
80/tcp open  http
```

Enumerated open TCP ports:

```bash
22/tcp open  ssh
80/tcp open  http
```

Enumerated top 200 UDP ports:

```bash

```

---

# Enumeration

## Port 80 - HTTP (nginx)

```
SSTI 

http://172.16.1.139/contact?name=%7B%7B7*7%7D%7D
```


---

# Exploitation

```
{{ self._TemplateReference__context.cycler.__init__.__globals__.os }}
```

## SQL Injection


---

# Lateral Movement to xxx

## Local enumeration


## Lateral movement vector

---

# Privilege Escalation to xxx

## Local enumeration


## Privilege Escalation vector


---

# Trophy

{{image}}

>[!todo] **User.txt**
>flag

>[!todo] **Root.txt**
>flag

**/etc/shadow**

```bash

```