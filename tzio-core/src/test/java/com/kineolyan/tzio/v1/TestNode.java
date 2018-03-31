package com.kineolyan.tzio.v1;

import com.kineolyan.tzio.v1.slot.InputSlot;
import com.kineolyan.tzio.v1.slot.OutputSlot;
import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.assertThat;

class TestNode {

	@Test
	public void testAccChanges() {
		final Node node = defaultNode();
		assertThat(node.getAccValue()).isEqualTo(0);

		node.setAccValue(10);
		assertThat(node.getAccValue()).isEqualTo(10);

		node.setAccValue(-7);
		assertThat(node.getAccValue()).isEqualTo(-7);
	}

	private Node defaultNode() {
		return new Node(1, new InputSlot[1], new OutputSlot[1]);
	}

}