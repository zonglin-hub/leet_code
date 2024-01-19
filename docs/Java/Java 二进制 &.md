## 二进制 &

```sh
public boolean isValid(String str) {
    int len = str.length();
    return (len & 1) != 1;
}
```

> **0000 0101**
>
> &
>
> **0000 0001**
>
> =
>
> **0000 0001**

## 10进制 %

```sh
public boolean isValid(String str) {
    int len = str.length();
    return len % 2 != 1;
}
```

> **10 % 2 == 0**
>
> **11 % 2 == 1**

‍
