# dApp 프로젝트 생성 및 지갑 연결

여러 지갑을 지원하고, 앱체인과 통신하는 cosmjs를 지원하는 cosmos-kit을 설치하고 프로젝트를 생성한다.
cosmos-kit template 프로젝트 생성 테스트

## cosmos-kit 설치

nodejs 설치 확인

```
$ node -v
> v20.11.1
# 버전은 맞출 필요는 없지만 지원이 안되는 경우 nvm을 사용하여 nodejs 버전을 변경하여 사용하도록 한다.
```

cosmos-kit 설치

```
$ npm i -g create-cosmos-app
> ... added 79 packages in 8s ...


# 설치 후 cca 명령어를 이용해 cosmos-kit 관련 작업을 진행한다.
$ cca -v
> 2.3.3
```

## cosmos-kit 프로젝트 생성 및 실행

```
# `cca` 또는 `create-cosmos-app`을 통해 프로젝트 생성을 한다.
$ cca

? [name] Enter your new app name rtd-dapp-m1
> Cloning into 'rtd-dapp-m1'...

? [template] which template connect-chain
> ✨ Have fun! Now you can start on your project ⚛️
> Now, run this command:
> cd ./rtd-dapp-m1 && yarn dev


# yarn을 입력시, yarn version / corepack 관련 오류가 나게 되면
# package.json에서 "packageManager": "yarn@4.3.1" 부분을 제거 후 실행한다
```

## 프로젝트 실행 및 확인

```
$ yarn dev
```

http://localhost:3000 주소로 테스트를 진행한다.

![m1-1](../../images/m1-1.png)

설치된 지갑을 찾아서 연결 및 주소를 확인한다.
![m1-2](../../images/m1-2.png)

## Multichain 프로젝트

cosmos-kit 생성시 template 를 connect-multi-chain으로 설정하여 프로젝트를 생성 후 실행한다.

![m1-3](../../images/m1-1.png)

![m1-4](../../images/m1-2.png)

여러 체인으로 계정이 변경 되는 것을 확인한다.(Cosmos 앱 체인은 체인별로 Address의 Prefix가 존재한다.)



https://cosmology.zone/products/create-cosmos-app