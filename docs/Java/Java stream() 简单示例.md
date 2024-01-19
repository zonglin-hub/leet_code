```java
public static void main(String[] args) {

    List<Integer> list = new ArrayList<>();
    list.add(1);
    list.add(2);
    list.add(3);
    list.add(4);
    list.add(4);
    list = list
            .stream()
            .distinct() // 去重
            .sorted((a, b) -> b - a)    // 进行倒序排列
            .map(e -> e + 1)    // 每个元素都要执行 +1 操作
            .limit(2)   // 只放行前两个元素
            .collect(Collectors.toList());

    System.out.println(list);
}
```
