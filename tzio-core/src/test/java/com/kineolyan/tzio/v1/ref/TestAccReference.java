package com.kineolyan.tzio.v1.ref;

import com.kineolyan.tzio.v1.Node;
import com.kineolyan.tzio.v1.slot.InputSlot;
import com.kineolyan.tzio.v1.slot.OutputSlot;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.assertThat;

class TestAccReference {

	private Node node;
	private AccReference ref;

	@BeforeEach
	void setUp() {
		this.node = new Node(1, new InputSlot[1], new OutputSlot[1]);
		this.ref = AccReference.INSTANCE;
	}

	@Test
	void testCanAlwaysRead() {
		assertThat(this.ref.canRead(this.node)).isTrue();
	}

	@Test
	void testCanAlwaysWrite() {
		assertThat(this.ref.canWrite(this.node)).isTrue();
	}

	@Test
	void testReadNodeValue() {
		this.node.setAccValue(12);
		assertThat(this.ref.readValue(this.node)).isEqualTo(12);
	}

	@Test
	void testWriteIntoNodeValue() {
		this.node.setAccValue(47);
		this.ref.writeValue(this.node, 5);
		assertThat(this.node.getAccValue()).isEqualTo(5);
	}

}