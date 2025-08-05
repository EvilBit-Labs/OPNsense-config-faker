from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Qnameminstrict(BaseModel):
    class Meta:
        name = "qnameminstrict"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
