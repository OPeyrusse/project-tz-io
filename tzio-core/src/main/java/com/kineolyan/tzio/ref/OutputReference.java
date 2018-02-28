package com.kineolyan.tzio.ref;

import com.kineolyan.tzio.Node;

public interface OutputReference {

	boolean canWrite(Node node);

	void writeValue(Node node, int value);

}
