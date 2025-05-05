//Credits to libTesseract: https://github.com/ollydev/libTesseract/blob/master/main.cpp
#include "exports.h"

#if defined(_WIN32) || defined(_WIN64)

HMODULE module = nullptr;

extern "C" EXPORT BOOL APIENTRY DllMain(HINSTANCE hinstDLL, DWORD fdwReason, LPVOID lpvReserved)
{
	module = hinstDLL;

	return TRUE;
}
#endif
