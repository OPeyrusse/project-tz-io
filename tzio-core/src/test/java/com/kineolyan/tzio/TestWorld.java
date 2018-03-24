package com.kineolyan.tzio;

import com.kineolyan.tzio.ops.MovOperation;
import com.kineolyan.tzio.ref.SlotReference;
import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

import static org.assertj.core.api.Assertions.assertThat;

public class TestWorld {

	@Test
	public void test() {
		final TzEnv env = new TzEnv()
			.withSlots(3, new int[] {0}, new int[] {2})
			.addNode(
				"a",
				1,
				new int[] {0},
				new int[] {1},
				List.of(
					new MovOperation(SlotReference.of(1), SlotReference.of(1))
				))
			.addNode(
				"b",
				1,
				new int[] {1},
				new int[] {2},
				List.of(
					new MovOperation(SlotReference.of(1), SlotReference.of(1))
				));

		final List<List<Integer>> outputs = new ArrayList<>();
		env.produceInto(values -> outputs.add(
			IntStream.of(values)
				.boxed()
				.collect(Collectors.toList())));
		env.consume(new int[] {1});
		env.consume(new int[] {2});
		while (outputs.size() < 2) {
			env.tick();
		}
		assertThat(outputs).containsExactly(List.of(1), List.of(2));
	}

}
