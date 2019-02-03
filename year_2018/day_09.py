import unittest


class TestMarbleGame(unittest.TestCase):
    def test_starts_empty(self):
        game = MarbleGame(0, 0)
        self.assertListEqual([], game.scores)
        self.assertListEqual([0], game._circle)

    def test_play_examples(self):
        def high_score(players, last_marble):
            game = MarbleGame(players, last_marble)
            return game.play()

        self.assertEqual(32, high_score(9, 25))
        self.assertEqual(8317, high_score(10, 1618))
        self.assertEqual(146373, high_score(13, 7999))
        self.assertEqual(2764, high_score(17, 1104))
        self.assertEqual(54718, high_score(21, 6111))
        self.assertEqual(37305, high_score(30, 5807))

    def test_play_mine(self):
        self.assertEqual(410375, MarbleGame(439, 71307).play())

    @unittest.skip("Too slow, > 2h")
    def test_play_huge_mine(self):
        self.assertEqual(3314195047, MarbleGame(439, 71307 * 100).play())


class MarbleGame:
    def __init__(self, num_players, last_marble):
        self.scores = [0 for _ in range(num_players)]
        self.last_marble = last_marble

        self._circle = [0]
        self._next_marble = 1
        self._current_index = 0

    def play(self):
        while self._next_marble <= self.last_marble:
            self.play_round()
        return max(self.scores)

    def play_round(self):
        if self._next_marble % 23 != 0:
            next_index = (self._current_index + 2) % len(self._circle)
            self._circle.insert(next_index, self._next_marble)
        else:
            num_players = len(self.scores)
            current_player_index = self._next_marble % num_players
            self.scores[current_player_index] += self._next_marble
            next_index = (self._current_index - 7) % len(self._circle)
            self.scores[current_player_index] += self._circle.pop(next_index)
        self._current_index = next_index
        self._next_marble += 1

    def print(self):
        print(" ".join(map(str, self.scores)), "  ",
              self._next_marble, "-",
              self._current_index, "-",
              " ".join(map(str, self._circle)))
        print()
