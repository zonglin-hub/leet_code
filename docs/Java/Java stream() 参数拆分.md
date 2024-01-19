```
    public static void main(String[] args) {
        List<String> list = new ArrayList<String>() {
            {
                this.add("a,b");
                this.add("c,d");
                this.add("e,f");
            }
        };
        System.out.println(list);   // [a,b, c,d, e,f]
        list = list
                .stream()
                .flatMap(e -> Arrays.stream(e.split(",")))
                .collect(Collectors.toList());
        System.out.println(list);   // [a, b, c, d, e, f]
    }
```