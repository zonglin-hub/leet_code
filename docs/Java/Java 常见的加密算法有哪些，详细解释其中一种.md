# 常见的加密算法有哪些，详细解释其中一种

常见的算法：DES、AES、RSA、Base64、MD5

```java
   public void testMd5() throws Exception {
       String[] attrs = {"0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f"};
       // MD5：单向加密算法，任何明文经过加密都是变成一个长度为32位的字符串
       MessageDigest md5 = MessageDigest.getInstance("MD5");
       // 明文密码
       String pwd = "1";
       // 字节的长度永远为16
       byte[] digest = md5.digest(pwd.getBytes("UTF-8"));
       // 用来记录明文加密后的密文字符串
       String miWen = "";
       for (byte b : digest) {
           int temp = b;
           if (temp < 0) {
               temp += 256;
           }
           int index1 = temp / 16;
           int index2 = temp % 16;
           miWen += attrs[index1] + attrs[index2];
       }
       System.out.println("加密后的密文字符串为：" + miWen);
       System.out.println("密文的字符个数为：" + miWen.length());
   }
```
