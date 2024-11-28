import unittest
from ext_types_custom2 import *

class TestGuid(unittest.TestCase):
    def test_get_guid(self):
        self.assertEqual(get_ouid2(None), "Ouid")


if __name__=='__main__':
    unittest.main()
