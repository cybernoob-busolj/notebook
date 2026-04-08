---
tags:
  - kubernetes/concepts/rbac-abac
---
-- --

### RBAC Overview (Role Based Access Control)

- A security model in k8s used to regulate access to cluster resources based on the **roles** assigned to users, group or service accounts
- It is formed by three major components:
	1. **Who**: The subject (user, group, or service account)
	2. **What**: The action they can perform (verbs)
	3. **Which**: The resource or API object
- It's centralized and declarative policy management
- Supports multi-tenant and shared cluster environments
- Enforces least privilege and users get only the permission they need

![[Pasted image 20260408104626.png]]


### ABAC Overview (Attribute Based Access Control)
- Access decisions are made on users, resource, and environment attributes
- Policies are written in JSON and evaluated dynamically
- Very flexible but hard to manage at scale

### Roles and Cluster Roles

**Roles**
- Roles are namespaced-scoped
- Roles control permissions within a single namespace
- Roles can only be bound within their namespace

**Cluster Roles**
- ClusterRoles are cluster-scoped or global
- ClusterRoles can control access to cluster-wide resources or all namespaces
- ClusterRoles can be bound at cluster level

### Subjects

- Subjects are entities that request access to kubernetes resources
- Types of subjects:
	- Users:
		- Authenticated human users (via OIDC, TLS certs)
	- Groups:
		- Logical grouping of users (e.g. devs, infra)
	- ServiceAccounts:
		- Identities for workloads (pods) within the cluster

```YAML
# YAML Example
subjects:
- kind: Users
  name: bob

- kind: Group
  name: developers

- kind: ServiceAccounts
  name: backend-sa
  namespaces: prod
```



---
### References

[kubernetes/reference/rbac](https://kubernetes.io/docs/reference/access-authn-authz/rbac/)
[kubernetes/reference/rbac-good-practices](https://kubernetes.io/docs/concepts/security/rbac-good-practices/)