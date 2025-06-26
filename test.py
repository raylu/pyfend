import unittest

import pyfend

class TestFend(unittest.TestCase):
	def test_evaluate(self):
		self.assertEqual(pyfend.evaluate('1+2'), '3')

	def test_error(self):
		with self.assertRaises(pyfend.FendError):
			pyfend.evaluate('1+')
