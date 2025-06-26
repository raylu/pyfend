import unittest

import pyfend

class TestFend(unittest.TestCase):
	def test_upper(self):
		self.assertEqual(pyfend.sum_as_string(1, 2), '3')
