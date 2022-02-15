#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

#ifndef ARRAY_SIZE
#define ARRAY_SIZE(a) (sizeof(a) / sizeof((a)[0]))
#endif

#define WIN_MINUTES  60
#define WIN_SECONDS (WIN_MINUTES * 60)
/* Number of slots in window array (worst case 2 slots per minute) */
#define WIN_SLOTS (WIN_MINUTES * 2)
/* Maximum height (meters) to assume when height value is not available */
#define MAX_HEIGHT 10000

struct day {
	char date[12];
	uint32_t min;
} days[365];

/* Meters to feet conversion */
static uint32_t mtof(uint32_t m)
{
	return (uint32_t)((double)m * 3.2808399);
}

static int entry_cmp(const void *a, const void *b)
{
	const uint32_t *entry1 = a;
	const uint32_t *entry2 = b;

	return (int)*entry1 - (int)*entry2;
}

static uint32_t new_min(uint32_t win[WIN_SLOTS], uint32_t slot_count, uint32_t old_min)
{
	if (slot_count == 0) {
		return old_min;
	}

	qsort(win, slot_count, sizeof(win[0]), entry_cmp);

	uint32_t min = win[slot_count / 2];
	if (min < old_min) {
		return min;
	}

	return old_min;
}

static bool win_complete(uint32_t win_time, uint32_t slot_count)
{
	return win_time > WIN_SECONDS || slot_count >= WIN_SLOTS;
}

int main(int argc, char *argv[])
{
	char date[16], time[16], type[16], height_str[16];
	char cur_date[16] = "";
	uint32_t day_count = 0;
	uint32_t slot = 0;
	uint32_t win_start = 0;
	uint32_t day_min = MAX_HEIGHT;
	struct day *day = NULL;
	uint32_t unix;

	while (true) {
		uint32_t win[WIN_SLOTS];
		uint32_t win_time;
		uint32_t height;

		if (fscanf(stdin, "%s %s %s %u.%*u %s",
			   date, time, type, &unix, height_str) != 5) {

			if (day && !win_complete(win_time, slot)) {
				strcpy(day->date, cur_date);
				day->min = day_min;
			}

			break;
		}

		if (win_start != 0) {
			win_time = unix - win_start;
		} else {
			win_time = 0;
		}

		if (strcmp(height_str, "None") == 0) {
			height = MAX_HEIGHT;
		} else {
			height = strtol(height_str, NULL, 0);
		}

		/* New day starting */
		if (strcmp(cur_date, date) != 0) {
			if (day) {
				/* Update statistics for previous day */
				if (!win_complete(win_time, slot)) {
					day_min = new_min(win, slot, day_min);
				}

				strcpy(day->date, cur_date);
				day->min = day_min;
			}

			strcpy(cur_date, date);
			slot = 0;
			day_min = MAX_HEIGHT;
			win_start = unix;

			if (day_count < ARRAY_SIZE(days)) {
				day = &days[day_count++];
			} else {
				day = NULL;
				printf("Too small days array!\n");
			}
		}

		if (slot == WIN_SLOTS) {
			if (win_time > WIN_SECONDS) {
				win_start = unix;
				slot = 0;
				win_time = 0;
			} else {
				printf("Too few window slots!\n");
				continue;
			}
		} else if (win_time > WIN_SECONDS) {
			day_min = new_min(win, slot, day_min);
			win_start = unix;
			slot = 0;
			win_time = 0;
		}

		if (win_time <= WIN_SECONDS && slot < WIN_SLOTS) {
			win[slot++] = height;

			if (slot == WIN_SLOTS) {
				day_min = new_min(win, slot, day_min);
				continue;
			}
		}
	}

	uint32_t below_2000 = 0, below_1000 = 0, below_500 = 0,
		 below_400 = 0, below_300 = 0, below_200 = 0;

	for (int i = 0; i < day_count; i++) {
		day = &days[i];

		uint32_t f = mtof(day->min);
		if (f < 2000) {
			below_2000++;
			if (f < 1000) {
				below_1000++;
				if (f < 500) {
					below_500++;
					if (f < 400) {
						below_400++;
						if (f < 300) {
							below_300++;
							if (f < 200) {
								below_200++;
							}
						}
					}
				}
			}
		}

		printf("%s %u\n", day->date, f);
	}

	printf("\n%u days total\n", day_count);
	printf("< 2000: %u, < 1000: %u, < 500: %u, < 400: %u, < 300: %u, < 200: %u\n",
	       below_2000, below_1000, below_500, below_400, below_300, below_200);

	return 0;
}
