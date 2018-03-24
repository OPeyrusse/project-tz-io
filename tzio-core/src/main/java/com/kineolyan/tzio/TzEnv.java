package com.kineolyan.tzio;

import com.kineolyan.tzio.ops.Operation;

import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.function.Consumer;
import java.util.function.IntFunction;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public class TzEnv {

	/** Output value marking the absence of value for the given output slot */
	public static int NO_OUTPUT = -1;

	private Map<String, NodeExecution> nodes;
	private Object[] slots;
	private InputQueueSlot[] inputs;
	private DataSlot[] outputs;
	private Consumer<int[]> consumer;

	public TzEnv() {
		this.nodes = new HashMap<>();
		this.consumer = values -> {};
	}

	public TzEnv withSlots(
		final int slotCount,
		final int[] inputs,
		final int[] outputs) {
		final Object[] slots = IntStream.range(0, slotCount)
			.mapToObj(i -> new DataSlot())
			.toArray();
		this.inputs = IntStream.of(inputs)
			.mapToObj(i -> {
				final InputQueueSlot inputSlot = new InputQueueSlot();
				// Replace the default slot
				slots[i] = inputSlot;
				return inputSlot;
			})
			.toArray(InputQueueSlot[]::new);
		this.outputs = getSlots(slots, outputs, DataSlot[]::new);
		this.slots = slots;
		return this;
	}

	public TzEnv addNode(
		final String name,
		final int memorySize,
		final int[] inputs,
		final int[] outputs,
		final List<Operation> operations) {
		final Node node = new Node(
			memorySize,
			getInputs(this.slots, inputs),
			getOutputs(this.slots, outputs));
		final NodeExecution execution = new NodeExecution(node, operations);

		final NodeExecution previousExecution = this.nodes.put(name, execution);
		if (previousExecution != null) {
			throw new IllegalStateException("Existing node registered under " + name);
		}
		return this;
	}

	public TzEnv produceInto(Consumer<int[]> consumer) {
		this.consumer = consumer;
		return this;
	}

	public void consume(final int[] input) {
		for (int i = 0; i < input.length; i += 1) {
			this.inputs[i].enqueue(input[i]);
		}
	}

	public void tick() {
		this.nodes.values().forEach(NodeExecution::runStep);
		// Check for an output
		if (Stream.of(this.outputs).anyMatch(DataSlot::canRead)) {
			final int[] output = Stream.of(this.outputs)
				.mapToInt(o -> o.canRead() ? o.read() : NO_OUTPUT)
				.toArray();
			this.consumer.accept(output);
		}
	}

	private static <T> T[] getSlots(final Object[] slots, final int[] indexes, final IntFunction<T[]> cstr) {
		return IntStream.of(indexes)
			.mapToObj(i -> (T) slots[i])
			.toArray(cstr);
	}

	private static InputSlot[] getInputs(final Object[] slots, final int[] inputs) {
		return getSlots(slots, inputs, InputSlot[]::new);
	}

	private static OutputSlot[] getOutputs(final Object[] slots, final int[] outputs) {
		return getSlots(slots, outputs, OutputSlot[]::new);
	}

}
