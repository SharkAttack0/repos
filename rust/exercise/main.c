#include <stddef.h>
#include <stdio.h>

typedef struct Node {
  int data;
  struct Node* next;
} Node;

int main(int argc, char *argv[])
{
  Node node1 = { .data = 32, .next = NULL };
  Node node2 = { .data = 20, .next = &node1 };
  Node node3 = { .data = 10, .next = &node2 };

  Node* iter = &node3;
  do {
    printf("%d\n", iter->data);
    iter = iter->next;
  } while(iter->next);
  return 0;
}
