#include <stdio.h>
#include <stdbool.h>

extern void console_output(bool debug, char *string, ...);

void console_output_str(bool debug, const char *msg)
{
    console_output(debug, "%s", msg);
}
