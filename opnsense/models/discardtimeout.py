from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Discardtimeout(BaseModel):
    class Meta:
        name = "discardtimeout"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
