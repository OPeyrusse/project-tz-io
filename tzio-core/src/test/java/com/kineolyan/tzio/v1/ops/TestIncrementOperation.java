package com.kineolyan.tzio.v1.ops;

import com.kineolyan.tzio.v1.Node;
import com.kineolyan.tzio.v1.ref.References;
import org.assertj.core.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

/**
 * @author Kineolyan
 */
public class TestIncrementOperation {

	private Node node;

	@BeforeEach
	void createNode() {
		this.node = OperationTestUtil.defaultNode();
	}

	@Test
	void testAddOperationWithValue() {
		this.node.setAccValue(54);
		final Operation.Shift shift = Operations.ADD(References.value(46)).execute(this.node);

		Assertions.assertThat(this.node.getAccValue()).isEqualTo(100);
		OperationTestUtil.assertThat(shift).shiftToNext();
	}

	@Test
	void testAddOperationWithAcc() {
		this.node.setAccValue(35);
		final Operation.Shift shift = Operations.ADD(References.acc()).execute(this.node);

		Assertions.assertThat(this.node.getAccValue()).isEqualTo(70);
		OperationTestUtil.assertThat(shift).shiftToNext();
	}

}
