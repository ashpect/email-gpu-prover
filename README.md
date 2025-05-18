# Prover

## Overview

This is a simple prover that can be used to prove a circuit. It is a simple Rust program that can be run on a Kubernetes cluster.

## Minikube

1. Install minikube
2. Install addons enable : metrics-enable for hpa 

```bash
minikube addons enable ingress
minikube addons enable metrics-server
(Try without adding on metrcs-server first, minikube become significantly slower otherwise)
```

3. Apply manifests (ns, utils, prover, hpa, api_microservice)
4. Get the minikube ip and hit on 3100 port

### Laundry List

- [X] Multiple api's support
- [X] Api microservice
- [X] Api expiration/usage limit
- [X] Metrics for api usage (time/number of requests)
- [X] Code changes for K8s minikube (without gpu using snarkjs)
- [X] Publicly hosted docker images for prover and api_microservice
- [X] Yaml manifests for everything
- [X] HPA's for autoscaling

Furthur improvements
- [ ] Microservice -> db for easier api keys management
- [ ] Helm charts for configurable deployments
