```
    public static void main(String[] args) {
        List<Integer> list = new ArrayList<Integer>() {
            {
                this.add(1);
                this.add(2);
                this.add(3);
            }
        };
        int num = list
                .stream()
                .reduce((a, b) -> a + b)
                .get();
        System.out.println(num);
    }
```