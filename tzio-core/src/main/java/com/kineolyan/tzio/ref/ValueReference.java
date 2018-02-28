package com.kineolyan.tzio.ref;

import com.kineolyan.tzio.Node;

import java.util.stream.IntStream;

public class ValueReference implements InputReference, OutputReference {

	private static ValueReference[] CACHE;
	private static int MIN_CACHED_VALUE = -128;
	private static int MAX_CACHED_VALUE = 128;
	static {
		CACHE = IntStream.range(MIN_CACHED_VALUE, MAX_CACHED_VALUE)
			.mapToObj(ValueReference::new)
			.toArray(ValueReference[]::new);
	}

	private final int value;

	public ValueReference(final int value) {
		this.value = value;
	}

	public static ValueReference of(final int value) {
		if (value >= -128 && value < 128) {
			return CACHE[value - MIN_CACHED_VALUE];
		} else {
			return new ValueReference(value);
		}
	}

	@Override
	public boolean canRead(Node node) {
		return true;
	}

	@Override
	public int readValue(final Node node) {
		return this.value;
	}

	@Override
	public boolean canWrite(Node node) {
		return true;
	}

	@Override
	public void writeValue(final Node node, final int value) {
		throw new RuntimeException(
			"Cannot write value into " + this + ". " +
			"Asked by " + node);
	}

	@Override
	public String toString() {
		return this.getClass().getSimpleName() + "{" +
			"value=" + value +
			'}';
	}
}
