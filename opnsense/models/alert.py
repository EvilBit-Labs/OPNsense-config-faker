from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.description import Description
from opnsense.models.enabled import Enabled
from opnsense.models.events import Events
from opnsense.models.format import Format
from opnsense.models.noton import Noton
from opnsense.models.recipient import Recipient
from opnsense.models.reminder import Reminder

__NAMESPACE__ = "https://opnsense.org/config"


class Alert(BaseModel):
    class Meta:
        name = "alert"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    enabled: Enabled = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    recipient: Recipient = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    noton: Noton = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    events: Events = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    format: Format = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    reminder: Reminder = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    description: Description = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    uuid: object = field(
        metadata={
            "type": "Attribute",
            "required": True,
        }
    )
