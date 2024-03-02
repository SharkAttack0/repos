int main() {
  int a = 2;
  printf("stack frame 1\n");
  printf("a:%u\n", &a);
  res();
  return 0;
}

void res() {
  printf("stack frame 2\n");
  int a = 2;
  int b = 3;
  int res = &b - &a;
  printf("&b - &a: %d\n", res);
  printf("&a:%u, &b:%u\n", &a, &b);

}
