import unittest

import pyfend

class TestFend(unittest.TestCase):
	def test_evaluate(self):
		self.assertEqual(pyfend.evaluate('1+2', pyfend.Context()), '3')

	def test_variable(self):
		context = pyfend.Context()
		pyfend.evaluate('a = 1 + 2', context)
		self.assertEqual(pyfend.evaluate('a + 3', context), '6')

	def test_random(self):
		pyfend.evaluate('roll 1d6', pyfend.Context())

	def test_error(self):
		with self.assertRaises(pyfend.FendError):
			pyfend.evaluate('1+', pyfend.Context())
