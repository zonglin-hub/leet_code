
```java
@GetMapping("/user/{id}")
public String hello3(@PathVariable int id) {
    return "根据id获取用户";
}

@PostMapping("/user")
public String hello4(User user) {
    return "添加用户";
}

@PutMapping("/user")
public String hello5(int id) {
    return "跟新用户";
}

@DeleteMapping("/user/{id}")
public String hello6(@PathVariable int id) {
    return "根据id删除用户";
}
```
