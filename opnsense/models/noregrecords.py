from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Noregrecords(BaseModel):
    class Meta:
        name = "noregrecords"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
