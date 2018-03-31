package com.kineolyan.tzio.v1.ops;

import com.kineolyan.tzio.v1.Node;
import com.kineolyan.tzio.v1.ref.InputReference;

import java.util.function.BiConsumer;

class ArithmeticOperation implements Operation {

	private final InputReference input;
	private final BiConsumer<Node, InputReference> consumer;

	public ArithmeticOperation(
		final InputReference input,
		final BiConsumer<Node, InputReference> consumer) {
		this.input = input;
		this.consumer = consumer;
	}

	@Override
	public Shift execute(Node node) {
		if (this.input.canRead(node)) {
			this.consumer.accept(node, this.input);
			return Shift.NEXT;
		} else {
			return Shift.STAY;
		}
	}

	public static ArithmeticOperation add(final InputReference input) {
		return new ArithmeticOperation(input, Node::addValue);
	}

	public static ArithmeticOperation sub(final InputReference input) {
		return new ArithmeticOperation(input, Node::subValue);
	}

}
