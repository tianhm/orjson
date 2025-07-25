# SPDX-License-Identifier: (Apache-2.0 OR MIT)

import pytest

import orjson


@pytest.mark.parametrize(
    "val",
    [
        b'"\xc8\x93',
        b'"\xc8',
    ],
)
def test_invalid(val):
    with pytest.raises(orjson.JSONDecodeError):
        orjson.loads(val)
