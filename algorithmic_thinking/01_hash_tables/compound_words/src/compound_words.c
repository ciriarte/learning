#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *read_line(unsigned int size) {
  char *str;
  char ch;
  unsigned int len = 0;
  str = malloc(size);
  if (str == NULL) {
    fprintf(stderr, "malloc error\n");
    exit(1);
  }

  while ((ch = (char)getchar()) != EOF && (ch != '\n')) {
    str[len++] = ch;
    if (len == size) {
      size = size * 2;
      str = realloc(str, size);
      if (str == NULL) {
        fprintf(stderr, "realloc error\n");
        exit(1);
      }
    }
  }
  str[len] = '\0';
  return str;
}

#define NUM_BITS 17

typedef struct word_node {
  char **word;
  struct word_node *next;
} word_node;

#define hashsize(n) ((unsigned long)1 << (n))
#define hashmask(n) (hashsize(n) - 1)

unsigned long oaat(char *key, size_t len,
                   size_t bits) {
  size_t hash, i;
  for (hash = 0, i = 0; i < len; i++) {
    hash += (unsigned long)key[i];
    hash += (hash << 10);
    hash ^= (hash >> 6);
  }

  hash += (hash << 3);
  hash ^= (hash >> 11);
  hash += (hash << 15);
  return hash & hashmask(bits);
}

int in_hash_table(word_node *hash_table[], char *find, size_t find_len) {
  unsigned long word_code;
  word_node *wordptr;
  word_code = oaat(find, find_len, NUM_BITS);
  wordptr = hash_table[word_code];
  while (wordptr) {
    if ((strlen(*(wordptr->word)) == find_len) &&
        (strncmp(*(wordptr->word), find, find_len) == 0))
        return 1;
    wordptr = wordptr->next;
  }
  return 0;
}

void identify_compound_words(char *words[],
                             word_node *hash_table[],
                             size_t total_words) {
  unsigned int i, j;
  size_t len;
  for (i = 0; i < total_words; i++) {
    len = strlen(words[i]);
    for (j = 1; j < len; j++) {
      if (in_hash_table(hash_table, words[i], j) &&
          in_hash_table(hash_table, &words[i][j], len - j)) {
        printf("%s\n", words[i]);
        break;
      }
    }
  }
}

#define WORD_LENGTH 16

int main(void){
  static char *words[1 << NUM_BITS] = { NULL };
  static word_node *hash_table[1 << NUM_BITS] = { NULL };
  size_t total = 0;
  char *word;
  word_node *wordptr;
  size_t length, word_code;
  word = read_line(WORD_LENGTH);
  while (*word) {
    words[total] = word;
    wordptr = malloc(sizeof(word_node));
    if (wordptr == NULL) {
      fprintf(stderr, "malloc error\n");
      exit(1);
    }
    length = strlen(word);
    word_code = oaat(word, length, NUM_BITS);
    wordptr->word = &words[total];
    wordptr->next = hash_table[word_code];
    hash_table[word_code] = wordptr;
    word = read_line(WORD_LENGTH);
    total++;
  }
  identify_compound_words(words, hash_table, total);
  return 0;
}
