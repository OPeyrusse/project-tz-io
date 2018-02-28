package com.kineolyan.tzio;

public interface TransactionalElement {

	default void onStepStart() {}

	default void onStepEnd() {}

}
