from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Depends(BaseModel):
    class Meta:
        name = "depends"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
