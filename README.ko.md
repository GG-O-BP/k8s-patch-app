# k8s-patch-app
소프트웨어 구현

## 다른 언어로 보기

[English](./README.md), [**한국어**](./README.ko.md)

# 설치
```
git clone https://github.com/GG-O-BP/k8s-patch-app.git
cd k8s-patch-app
cargo build --release
```

# 사용
```
cd k8s-patch-app
cd target
cd release
./k8s-patch-app -n=<deployment-name> -p='[{\"op\": \"replace\", \"path\": \"/spec/replicas\", \"value\":<new-replica-count>}]'
```

## 설명
-n에는 deployment 대상에 대한 이름을 입력 받습니다.

-p에는 k8s json patch에 대한 json 포맷을 입력 받습니다.

<deployment-name>에는 deployment 대상에 대한 이름을 입력해주세요.

<new-replica-count>에는 변경 할 replicas의 count를 입력해주세요.

ex) ./k8s-patch-app -n=test-deployment -p='[{\"op\": \"replace\", \"path\": \"/spec/replicas\", \"value\":3}]'
