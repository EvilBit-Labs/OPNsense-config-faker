from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.column_count import ColumnCount
from opnsense.models.sequence import Sequence

__NAMESPACE__ = "https://opnsense.org/config"


class Widgets(BaseModel):
    class Meta:
        name = "widgets"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    sequence: Sequence = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    column_count: ColumnCount = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
