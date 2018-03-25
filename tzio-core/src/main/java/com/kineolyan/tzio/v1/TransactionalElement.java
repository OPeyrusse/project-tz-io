package com.kineolyan.tzio.v1;

public interface TransactionalElement {

	default void onStepStart() {}

	default void onStepEnd() {}

}
