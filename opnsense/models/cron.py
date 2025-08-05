from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.jobs import Jobs

__NAMESPACE__ = "https://opnsense.org/config"


class Cron(BaseModel):
    class Meta:
        name = "cron"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    jobs: Jobs = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    version: str = field(
        metadata={
            "type": "Attribute",
            "required": True,
        }
    )
