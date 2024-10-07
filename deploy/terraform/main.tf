provider "aws" {
  region = "us-west-2"
}

resource "aws_instance" "windexer" {
  ami           = "ami-0c55b159cbfafe1f0"
  instance_type = "t2.micro"

  tags = {
    Name = "windexer"
  }
}

resource "aws_security_group" "windexer" {
  name        = "windexer"
  description = "Security group for Windexer"

  ingress {
    from_port   = 8080
    to_port     = 8080
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port   = 9100
    to_port     = 9100
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
}