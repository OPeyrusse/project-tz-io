package com.kineolyan.tzio.v1.ref;

import com.kineolyan.tzio.v1.Node;
import com.kineolyan.tzio.v1.slot.InputSlot;
import com.kineolyan.tzio.v1.slot.OutputSlot;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.assertThat;

class TestValueReference {

	private Node node;

	@BeforeEach
	void setUp() {
		this.node = new Node(1, new InputSlot[0], new OutputSlot[0]);
	}

	@Test
	void testCanAlwaysRead() {
		final ValueReference ref = ValueReference.of(12);
		assertThat(ref.canRead(this.node)).isTrue();
		ref.readValue(this.node);
		assertThat(ref.canRead(this.node)).isTrue();
	}

	@Test
	void testReadValue() {
		final ValueReference ref = ValueReference.of(124);
		assertThat(ref.readValue(this.node)).isEqualTo(124);
	}

}