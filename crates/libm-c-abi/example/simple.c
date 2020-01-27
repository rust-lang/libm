/* 
* Simple test program.
* You can use it to debug test case outside the test suite.
* This is usefull to debug in parallel the C and rust version, to float debug precision issue.
*/
#include <math.h>
#include <stdio.h>

int main(void)
{
    double res = y0(0x1.c982eb8d417eap-1);
    double want = -0x1.af74bfa0f1304p-56;
    double got = -0x1p-55;
    printf("want: %a : got : %a\ngot in test : %a\n", want, res, got);
}