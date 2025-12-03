package day01;
import java.util.ArrayList;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.concurrent.atomic.AtomicInteger;

public class day01 {
    public static void main(String[] args) throws IOException {
        ArrayList<String> lines; 
        try {
            lines = Files.lines(Path.of("data/day01.txt")).collect(ArrayList::new, ArrayList::add, ArrayList::addAll);
        } catch(IOException e) {
            throw e;
        }

        System.out.println("p1: " + part1(lines));
        System.out.println("p2: " + part2(lines));
    }


    private static int part1(ArrayList<String> data) {
        AtomicInteger dial = new AtomicInteger(50);
        return (int) data.stream().mapToInt((str) -> {
            Integer num = Integer.parseInt(str.substring(1));

            if (str.charAt(0) == 'L') {
                return -num;
            }
            return num;
        })
        .map((delta) -> {
            int current = dial.get() + delta;

            while (current > 99) {
                current = current - 100;
            }

            while (current < 0) {
                current += 100;
            }
            dial.set(current);
            return current;
        })
        .filter(num -> num == 0).count();
    }

    private static int part2(ArrayList<String> data) {
        AtomicInteger dial = new AtomicInteger(50);

        return data.stream().mapToInt((str) -> {
            Integer num = Integer.parseInt(str.substring(1));

            if (str.charAt(0) == 'L') {
                return -num;
            }
            return num;
        })
        .map((delta) -> {
            int prev = dial.get();
            int cycles = 0;

            int current = prev + delta;

            while (current < 0) {
                current += 100;
                cycles ++;
            }

            while (current > 99) {
                current -= 100;
                cycles ++;
            }

            dial.set(current);
            return cycles;
        }).sum();
    }
}
