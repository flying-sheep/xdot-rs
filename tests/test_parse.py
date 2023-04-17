import pytest
import xdot_rs
from xdot_rs.draw import Pen
from xdot_rs.shapes import Ellipse


def test_import_structure():
    assert callable(xdot_rs.parse)  # not a FunctionType?
    # TODO: assert xdot_rs.parse.__module__ == "xdot_rs"
    assert isinstance(xdot_rs.ShapeDraw, type)
    assert xdot_rs.ShapeDraw.__module__ == "xdot_rs"


@pytest.mark.parametrize("input", ["e 27 90 18 3"])
def test_parse(input):
    [sd] = xdot_rs.parse(input)
    assert (sd.shape.x, sd.shape.y) == (27.0, 90.0)
    assert (sd.shape.w, sd.shape.h) == (18.0, 3.0)
    assert sd.shape == (ell := Ellipse(27.0, 90.0, 18.0, 3.0, filled=False))
    assert sd.pen == Pen()
    assert sd == xdot_rs.ShapeDraw(ell, Pen())


# TODO: test that pens influence comparison


def test_parse_error():
    with pytest.raises(ValueError, match=r"error Tag"):
        xdot_rs.parse("")
