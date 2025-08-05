from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.from_mod import From
from opnsense.models.to import To

__NAMESPACE__ = "https://opnsense.org/config"


class Range(BaseModel):
    class Meta:
        name = "range"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    from_value: From = field(
        metadata={
            "name": "from",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    to: To = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
