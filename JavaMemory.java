public class JavaMemory {
    public static void main(String[] args) {
        Runtime runtime = Runtime.getRuntime();
        System.out.println("Total Memory: " + runtime.totalMemory());
        System.out.println("Free Memory before allocation: " + runtime.freeMemory());

        int[] largeArray = new int[10_000_000]; // Large heap allocation
        System.out.println("Free Memory after allocation: " + runtime.freeMemory());

        largeArray = null;  // Allow garbage collection
        System.gc(); // Suggest garbage collection
        System.out.println("Free Memory after GC: " + runtime.freeMemory());
    }
}
