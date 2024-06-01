# 02. App chain architecture

이 그림을 반드시 이해야함. 매우 중요..

![alt text](image.png)

```
                ^  +-------------------------------+  ^
                |  |                               |  |   Built with Cosmos SDK
                |  |  State-machine = Application  |  |
                |  |                               |  v
                |  +-------------------------------+
                |  |                               |  ^
Blockchain node |  |           Consensus           |  |
                |  |                               |  |
                |  +-------------------------------+  |   CometBFT
                |  |                               |  |
                |  |           Networking          |  |
                |  |                               |  |
                v  +-------------------------------+  v
```

https://youtu.be/1_ottIKPfI4?si=XstKA2YGi2-yYKzF
