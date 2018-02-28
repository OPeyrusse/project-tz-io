package com.kineolyan.tzio.ref;

import com.kineolyan.tzio.Node;

public interface InputReference {

	boolean canRead(Node node);

	int readValue(Node node);

}
