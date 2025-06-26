import unittest

import pyfend

class TestFend(unittest.TestCase):
	def test_evaluate(self):
		self.assertEqual(pyfend.evaluate('1+2'), '3')
