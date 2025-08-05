from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Serveraddress(BaseModel):
    class Meta:
        name = "serveraddress"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
