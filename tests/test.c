#include "../oppai/oppai.c"
#include "../oppai/test/test_suite.c"
#include <stdlib.h>

size_t size() {
	return sizeof(suite) / sizeof(suite[0]);
}

score_t *tests() {
	return suite;
}
