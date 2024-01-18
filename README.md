# k8s-patch-app
Software Implementation

## View in other languages

[**English**](./README.md), [한국어](./README.ko.md)

# Setup
```
git clone https://github.com/GG-O-BP/k8s-patch-app.git
cd k8s-patch-app
cargo build --release
```

# Usage
```
cd k8s-patch-app
cd target
cd release
./k8s-patch-app -n=<deployment-name> -p='[{\"op\": \"replace\", \"path\": \"/spec/replicas\", \"value\":<new-replica-count>}]'
```

## Notes
-n is used to input the name of the deployment target.
-p is used to input the json format for the k8s json patch.

Please input the name of the deployment target for <deployment-name>.
Please input the count of replicas to be changed for <new-replica-count>.

ex) ./k8s-patch-app -n=test-deployment -p='[{\"op\": \"replace\", \"path\": \"/spec/replicas\", \"value\":3}]'
